# telegram-id-bot

A clean Rust Telegram bot that analyzes user and forwarded message information with detailed ID tracking and account age estimation.

## Features

- Shows detailed information about users who contact the bot
- Analyzes forwarded messages and shows original sender information
- Estimates account age based on user ID using historical data
- Handles different types of forwards (user, chat, channel, hidden user)

## Setup

1. Install Rust if you haven't already

2. Set the required environment variables:
   ```bash
   export TELOXIDE_TOKEN="5668107317:AAGvMk3FxaA5ZGPFc4yYl6zHlksT8CJSDOg"
   export WEBHOOK_URL="https://yourdomain.com/webhook"
   export PORT="3000"  # Optional, defaults to 3000
   export HOST="0.0.0.0"  # Optional, defaults to 0.0.0.0
   ```

3. Build and run the bot:
   ```bash
   cargo build
   cargo run
   ```

   Or run directly with all environment variables:
   ```bash
   TELOXIDE_TOKEN="5668107317:AAGvMk3FxaA5ZGPFc4yYl6zHlksT8CJSDOg" \
   WEBHOOK_URL="https://yourdomain.com/webhook" \
   PORT="3000" \
   cargo run
   ```

## Webhook Setup

The bot now uses webhooks instead of polling for better performance and reliability.

### Environment Variables

- `TELOXIDE_TOKEN`: Your Telegram bot token (required)
- `WEBHOOK_URL`: Your public webhook URL (required, must be HTTPS)
- `PORT`: Port to run the web server on (optional, default: 3000)
- `HOST`: Host to bind to (optional, default: 0.0.0.0)

### Endpoints

- `GET /`: Health check endpoint
- `GET /health`: Health check endpoint
- `POST /webhook`: Telegram webhook endpoint

### Deployment

For production deployment, you'll need:
1. A public domain with HTTPS
2. Set `WEBHOOK_URL` to `https://yourdomain.com/webhook`
3. Ensure your server is accessible from the internet
4. Telegram will send updates to your webhook endpoint

## Usage

- Send `/start` to get a welcome message
- Send `/help` to see available commands  
- Send any message to see your user information
- Forward any message to see both your info and the original sender's info

## Bot Token

The bot token is: `5668107317:AAGvMk3FxaA5ZGPFc4yYl6zHlksT8CJSDOg`

Make sure to set it as the `TELOXIDE_TOKEN` environment variable before running the bot.
