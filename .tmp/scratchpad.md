355454548799717382

TOKEN="355454548799717382"
  API="http://localhost:3001"

  curl -X POST "$API/api/v1/prompt-loop" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{
      "prompt": "Explain semantic Ralph loops briefly"
    }'


curl -X POST http://localhost:8080/oauth/v2/token \
      -H "Content-Type: application/x-www-form-urlencoded" \
      -d
  "grant_type=client_credentials&client_id=YOUR_CLIENT_ID&client_secret=YOUR_CLI
  ENT_SECRET&scope=openid profile"
