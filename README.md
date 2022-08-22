# Telegram bot api consummer

Example of application which allows subscribers to consume API from other services.

## Commands
- /start - to start bot
- /letmein $username $password - to authorize (don't forget to pass username and password)
- /consumers - shows list of active consumers
- /vehicles $limit $offset - returns response from api (currently in progress)

## Installaion

1. Clone repository
2. Create telegram bot
3. Fill .env file with variables (check example .env.example)
4. Run ```docker-comopose up -d```
5. Check bot workability


## What can be done further?
- Client interface should be implement (buttons);
- More commands should be added;
- To add WS consumer to send failed messages from nats;
- To cover application with unit and integration tests;
- To set up precommit/prepush hooks;
- To make "code cleaning" - remove warnings, unused imports etc;
