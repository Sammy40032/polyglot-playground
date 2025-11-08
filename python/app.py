
from flask import Flask, jsonify, request

app = Flask(__name__)

@app.route('/hello', methods=['GET'])
def hello():
    return jsonify(message="Hello from Python!")

@app.route('/fibonacci', methods=['GET'])
def fibonacci():
    n = int(request.args.get('n', 10))
    a, b = 0, 1
    seq = []
    for _ in range(n):
        seq.append(a)
        a, b = b, a + b
    return jsonify(sequence=seq)

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
