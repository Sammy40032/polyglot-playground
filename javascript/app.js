
const express = require('express');
const app = express();
const port = 5001;

app.get('/hello', (req, res) => {
    res.json({ message: 'Hello from JavaScript!' });
});

app.get('/fibonacci', (req, res) => {
    const n = parseInt(req.query.n) || 10;
    let a = 0, b = 1;
    const seq = [];
    for (let i = 0; i < n; i++) {
        seq.push(a);
        [a, b] = [b, a + b];
    }
    res.json({ sequence: seq });
});

app.listen(port, () => console.log(`JavaScript API running on port ${port}`));
