const https = require('https');

// Age estimation data
const ageData = {
    2768409: 1383264000000,
    7679610: 1388448000000,
    11538514: 1391212000000,
    15835244: 1392940000000,
    23646077: 1393459000000,
    38015510: 1393632000000,
    44634663: 1399334000000,
    46145305: 1400198000000,
    54845238: 1411257000000,
    63263518: 1414454000000,
    101260938: 1425600000000,
    101323197: 1426204000000,
    111220210: 1429574000000,
    103258382: 1432771000000,
    103151531: 1433376000000,
    116812045: 1437696000000,
    122600695: 1437782000000,
    109393468: 1439078000000,
    112594714: 1439683000000,
    124872445: 1439856000000,
    130029930: 1441324000000,
    125828524: 1444003000000,
    133909606: 1444176000000,
    157242073: 1446768000000,
    143445125: 1448928000000,
    148670295: 1452211000000,
    152079341: 1453420000000,
    171295414: 1457481000000,
    181783990: 1460246000000,
    222021233: 1465344000000,
    225034354: 1466208000000,
    278941742: 1473465000000,
    285253072: 1476835000000,
    294851037: 1479600000000,
    297621225: 1481846000000,
    328594461: 1482969000000,
    337808429: 1487707000000,
    341546272: 1487782000000,
    352940995: 1487894000000,
    369669043: 1490918000000,
    400169472: 1501459000000,
    805158066: 1563208000000,
    1974255900: 1634000000000
};

function getAgeEstimate(userId) {
    const sortedIds = Object.keys(ageData).map(Number).sort((a, b) => a - b);
    const minId = sortedIds[0];
    const maxId = sortedIds[sortedIds.length - 1];
    
    if (userId < minId) {
        const date = new Date(ageData[minId]);
        return ["older_than", `${date.getMonth() + 1}/${date.getFullYear()}`];
    } else if (userId > maxId) {
        const date = new Date(ageData[maxId]);
        return ["newer_than", `${date.getMonth() + 1}/${date.getFullYear()}`];
    } else {
        let lid = sortedIds[0];
        for (const nid of sortedIds) {
            if (userId <= nid) {
                const lage = ageData[lid];
                const uage = ageData[nid];
                
                const idRatio = (userId - lid) / (nid - lid);
                const midDate = idRatio * (uage - lage) + lage;
                
                const date = new Date(midDate);
                return ["approx", `${date.getMonth() + 1}/${date.getFullYear()}`];
            } else {
                lid = nid;
            }
        }
    }
    
    return ["unknown", "unknown"];
}

function formatUserInfo(user, title) {
    let info = `👤 ${title}\n`;
    info += ` ├ id: ${user.id}\n`;
    info += ` ├ is_bot: ${user.is_bot ? 'true' : 'false'}\n`;
    info += ` ├ first_name: ${user.first_name}\n`;
    
    if (user.last_name) {
        info += ` ├ last_name: ${user.last_name}\n`;
    }
    
    if (user.username) {
        info += ` ├ username: ${user.username}\n`;
    }
    
    if (user.language_code) {
        info += ` ├ language_code: ${user.language_code} (-)\n`;
    }
    
    const [ageType, date] = getAgeEstimate(user.id);
    info += ` └ created: ${ageType} ${date} (?)\n`;
    
    return info;
}

function formatChatInfo(chat) {
    let info = "💬 Chat\n";
    info += ` ├ id: ${chat.id}\n`;
    
    const chatType = chat.type;
    
    if (chat.title) {
        info += ` ├ type: ${chatType}\n`;
        info += ` ├ title: ${chat.title}\n`;
        if (chat.username) {
            info += ` └ username: ${chat.username}\n`;
        } else {
            info = info.replace(" ├ title:", " └ title:");
        }
    } else {
        if (chat.username) {
            info += ` ├ type: ${chatType}\n`;
            info += ` └ username: ${chat.username}\n`;
        } else {
            info += ` └ type: ${chatType}\n`;
        }
    }
    
    return info;
}

async function sendTelegramMessage(chatId, text) {
    const token = process.env.TELOXIDE_TOKEN;
    const url = `https://api.telegram.org/bot${token}/sendMessage`;
    
    const payload = JSON.stringify({
        chat_id: chatId,
        text: text,
        parse_mode: 'HTML'
    });
    
    return new Promise((resolve, reject) => {
        const req = https.request(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Content-Length': Buffer.byteLength(payload)
            }
        }, (res) => {
            let data = '';
            res.on('data', chunk => data += chunk);
            res.on('end', () => {
                if (res.statusCode >= 200 && res.statusCode < 300) {
                    resolve(JSON.parse(data));
                } else {
                    reject(new Error(`HTTP ${res.statusCode}: ${data}`));
                }
            });
        });
        
        req.on('error', reject);
        req.write(payload);
        req.end();
    });
}

module.exports = async (req, res) => {
    // Set CORS headers
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'POST, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

    // Handle preflight requests
    if (req.method === 'OPTIONS') {
        return res.status(200).end();
    }

    // Only handle POST requests for webhooks
    if (req.method !== 'POST') {
        return res.status(405).json({ error: 'Method not allowed' });
    }

    try {
        const update = req.body;
        
        if (update.message) {
            const message = update.message;
            const chatId = message.chat.id;
            
            // Check if it's a command
            if (message.text && message.text.startsWith('/start')) {
                const user = message.from;
                let welcomeText = `Hi ${user.first_name}!\n\n`;
                welcomeText += "🤖 Telegram ID Bot\n\n";
                welcomeText += "How this bot works:\n";
                welcomeText += "• Send me any message to see your detailed user information\n";
                welcomeText += "• Forward any message to me to see both your info and the original sender's details\n";
                welcomeText += "• I can estimate account creation dates based on user IDs\n";
                welcomeText += "• All information is displayed in a clean tree format\n\n";
                welcomeText += "Try sending me a message or forwarding one to see it in action!";
                
                await sendTelegramMessage(chatId, welcomeText);
            } else if (message.text && message.text.startsWith('/help')) {
                const helpText = "Available commands:\n/start - Start the bot\n/help - Show this help message";
                await sendTelegramMessage(chatId, helpText);
            } else {
                // Regular message or forwarded message
                let response = '';
                
                if (message.from) {
                    response += formatUserInfo(message.from, "You");
                }
                
                response += "\n";
                response += formatChatInfo(message.chat);
                
                if (message.forward_from) {
                    response += "\n";
                    response += formatUserInfo(message.forward_from, "Forwarded from");
                    
                    response += "\n📃 Message\n";
                    if (message.forward_date) {
                        const date = new Date(message.forward_date * 1000);
                        response += ` └ forward_date: ${date.toUTCString()}\n`;
                    }
                }
                
                await sendTelegramMessage(chatId, response);
            }
        }

        return res.status(200).json({ ok: true });

    } catch (error) {
        console.error('Webhook error:', error);
        return res.status(500).json({ error: 'Internal server error' });
    }
};