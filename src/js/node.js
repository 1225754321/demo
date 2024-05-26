class Vec {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
    plus(other) {
        return new Vec(this.x + other.x, this.y + other.y);
    }
    minus(other) {
        return new Vec(this.x - other.x, this.y - other.y);
    }
    times(factor) {
        return new Vec(this.x * factor, this.y * factor);
    }
    get length() {
        return Math.sqrt(this.x * this.x + this.y * this.y);
    }
}

class GraphNode {
    constructor() {
        this.pos = new Vec(Math.random() * 1000, Math.random() * 1000);
        this.edges = [];
    }

    connect(other) {
        this.edges.push(other);
        other.edges.push(this);
    }

    hasEdge(other) {
        return this.hasEdgeFast(other)
    }

}

GraphNode.prototype.hasEdgeFast = function (other) {
    for (let i = 0; i < this.edges.length; i++) {
        if (this.edges[i] === other) return true
    }
    return false;
}
const springLength = 40; //弹簧静止长度
const springStrength = 0.1; //弹性系数
const repulsionStrength = 1500; //库伦常数
const skipDistance = 175; // 距离过远跳过计算

function forceDirected_noRepeat_skip_noVector(graph) {
    for (let i = 0; i < graph.length; i++) {
        let node = graph[i];
        for (let j = i + 1; j < graph.length; j++) {
            let other = graph[j];

            let apartX = other.pos.x - node.pos.x;
            let apartY = other.pos.y - node.pos.y;
            let distance = Math.max(1, Math.sqrt(apartX * apartX + apartY * apartY))

            let hasEdge = node.hasEdge(other);
            if (!hasEdge && distance > skipDistance) continue;
            let forceSize = -repulsionStrength / (distance * distance);
            if (node.hasEdge(other)) {
                forceSize += (distance - springLength) * springStrength;
            }

            let forceX = apartX * forceSize / distance;
            let forceY = apartY * forceSize / distance;

            node.pos.x += forceX;
            node.pos.y += forceY;

            other.pos.x -= forceX;
            other.pos.y -= forceY;

        }
    }
}
