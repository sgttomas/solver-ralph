"""
Prompt Registry for Chirality Framework

Loads and validates maintainer-authored prompt assets from metadata.yml.
Provides access to versioned prompt assets with integrity checking.
"""

import hashlib
import yaml
from pathlib import Path
from typing import Dict, Optional
from dataclasses import dataclass


@dataclass
class AssetInfo:
    """Information about a prompt asset."""

    id: str
    path: str
    sha256: str
    version: str
    size_bytes: int
    last_modified: str
    text: str  # Loaded content


class PromptRegistry:
    """
    Registry for maintainer-authored prompt assets.

    Loads metadata.yml and validates each asset against its SHA256 and size.
    Caches asset contents in memory for performance.
    """

    def __init__(self, assets_dir: Optional[Path] = None):
        """
        Initialize registry with assets directory.

        Args:
            assets_dir: Path to prompt assets directory.
                       Defaults to chirality/infrastructure/prompts/assets/
        """
        if assets_dir is None:
            # Default to assets/ relative to this file (in infrastructure/prompts/)
            current_dir = Path(__file__).parent
            assets_dir = current_dir / "assets"

        self.assets_dir = Path(assets_dir)
        self.metadata_file = self.assets_dir / "metadata.yml"
        self._assets: Dict[str, AssetInfo] = {}
        self._loaded = False

    def load(self) -> None:
        """Load and validate all assets from metadata.yml."""
        if not self.metadata_file.exists():
            raise FileNotFoundError(f"Metadata file not found: {self.metadata_file}")

        # Load metadata
        with open(self.metadata_file, "r", encoding="utf-8") as f:
            metadata = yaml.safe_load(f)

        registry_version = metadata.get("registry_version", "1.0")
        if registry_version != "1.0":
            raise ValueError(f"Unsupported registry version: {registry_version}")

        # Load each asset
        for asset_data in metadata.get("assets", []):
            asset_id = asset_data["id"]
            asset_path = self.assets_dir / asset_data["path"]

            # Validate file exists
            if not asset_path.exists():
                raise FileNotFoundError(f"Asset file not found: {asset_path}")

            # Load content
            with open(asset_path, "r", encoding="utf-8") as f:
                content = f.read()

            # Validate SHA256 (skip for pending assets during development)
            actual_sha256 = hashlib.sha256(content.encode("utf-8")).hexdigest()
            expected_sha256 = asset_data["sha256"]
            if expected_sha256 != "pending_user_authoring" and actual_sha256 != expected_sha256:
                raise ValueError(
                    f"SHA256 mismatch for {asset_id}: "
                    f"expected {expected_sha256}, got {actual_sha256}"
                )

            # Validate size (skip for pending assets)
            actual_size = len(content.encode("utf-8"))
            expected_size = asset_data.get("size_bytes")
            if expected_size is not None and actual_size != expected_size:
                raise ValueError(
                    f"Size mismatch for {asset_id}: "
                    f"expected {expected_size} bytes, got {actual_size} bytes"
                )

            # Store asset info
            self._assets[asset_id] = AssetInfo(
                id=asset_id,
                path=asset_data["path"],
                sha256=expected_sha256,
                version=asset_data["version"],
                size_bytes=expected_size,
                last_modified=asset_data.get("last_modified"),
                text=content,
            )

        self._loaded = True

    def get(self, asset_id: str) -> AssetInfo:
        """
        Get asset by ID.

        Args:
            asset_id: Asset identifier (e.g., 'system', 'station.evaluation')

        Returns:
            AssetInfo with loaded text content

        Raises:
            KeyError: If asset_id not found
            RuntimeError: If registry not loaded
        """
        if not self._loaded:
            self.load()

        if asset_id not in self._assets:
            available = list(self._assets.keys())
            raise KeyError(f"Asset '{asset_id}' not found. Available: {available}")

        return self._assets[asset_id]

    def get_text(self, asset_id: str) -> str:
        """Get asset text content directly."""
        return self.get(asset_id).text

    def list_assets(self) -> Dict[str, str]:
        """
        List all available assets.

        Returns:
            Dict mapping asset_id -> version
        """
        if not self._loaded:
            self.load()

        return {aid: asset.version for aid, asset in self._assets.items()}

    def get_provenance(self, asset_id: str) -> Dict[str, str]:
        """
        Get provenance information for an asset.

        Returns:
            Dict with id, sha256, version for provenance logging
        """
        asset = self.get(asset_id)
        return {"id": asset.id, "sha256": asset.sha256, "version": asset.version}

    def compute_kernel_hash(self, normative_spec_path: Optional[Path] = None) -> str:
        """
        Compute kernel hash from all assets + normative spec.

        Args:
            normative_spec_path: Path to normative_spec.txt

        Returns:
            SHA256 kernel hash
        """
        if not self._loaded:
            self.load()

        # Collect asset hashes in sorted order
        asset_hashes = []
        for asset_id in sorted(self._assets.keys()):
            asset_hashes.append(self._assets[asset_id].sha256)

        # Get normative spec hash
        if normative_spec_path and normative_spec_path.exists():
            normative_bytes = normative_spec_path.read_bytes()
            normative_hash = hashlib.sha256(normative_bytes).hexdigest()
        else:
            # Default path relative to chirality root
            current_dir = Path(__file__).parent.parent.parent
            default_spec_path = current_dir / "normative_spec.txt"
            if default_spec_path.exists():
                normative_bytes = default_spec_path.read_bytes()
                normative_hash = hashlib.sha256(normative_bytes).hexdigest()
            else:
                normative_hash = "missing"

        # Compute kernel hash
        return self._compute_kernel_hash(asset_hashes, normative_hash)

    def _compute_kernel_hash(self, asset_hashes: list[str], normative_hash: str) -> str:
        """
        Compute kernel hash from components.

        Args:
            asset_hashes: List of asset SHA256 hashes
            normative_hash: Normative spec SHA256 hash

        Returns:
            Combined SHA256 kernel hash
        """
        h = hashlib.sha256()

        # Add sorted asset hashes
        for asset_hash in sorted(asset_hashes):
            h.update(asset_hash.encode("utf-8"))

        # Add normative spec hash
        h.update(normative_hash.encode("utf-8"))

        return h.hexdigest()

    def create_manifest(
        self, output_path: Path, normative_spec_path: Optional[Path] = None
    ) -> Dict[str, str]:
        """
        Create asset manifest with checksums and kernel hash.

        Args:
            output_path: Path to write manifest YAML
            normative_spec_path: Optional path to normative spec

        Returns:
            Dict with kernel_hash and asset_count
        """
        if not self._loaded:
            self.load()

        kernel_hash = self.compute_kernel_hash(normative_spec_path)

        # Build manifest
        from datetime import datetime, timezone

        manifest = {
            "generated_at": datetime.now(timezone.utc).isoformat(),
            "kernel_hash": kernel_hash,
            "asset_count": len(self._assets),
            "assets": {
                aid: {
                    "id": asset.id,
                    "path": asset.path,
                    "sha256": asset.sha256,
                    "version": asset.version,
                    "size_bytes": asset.size_bytes,
                    "last_modified": asset.last_modified,
                }
                for aid, asset in self._assets.items()
            },
        }

        # Write manifest
        output_path = Path(output_path)
        output_path.parent.mkdir(parents=True, exist_ok=True)

        with open(output_path, "w") as f:
            yaml.dump(manifest, f, default_flow_style=False, sort_keys=True)

        return {"kernel_hash": kernel_hash, "asset_count": len(self._assets)}


# Global registry instance
_registry: Optional[PromptRegistry] = None


def get_registry() -> PromptRegistry:
    """Get the global prompt registry instance."""
    global _registry
    if _registry is None:
        _registry = PromptRegistry()
    return _registry
