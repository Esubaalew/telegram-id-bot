module.exports = async (req, res) => {
    // Set CORS headers
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

    // Handle preflight requests
    if (req.method === 'OPTIONS') {
        return res.status(200).end();
    }

    if (req.method === 'GET') {
        return res.status(200).json({
            status: 'healthy',
            service: 'Telegram ID Analyzer Bot',
            timestamp: new Date().toISOString(),
            version: '1.0.0',
            runtime: 'Node.js on Vercel'
        });
    }

    return res.status(405).json({ error: 'Method not allowed' });
};