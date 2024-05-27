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

    hasEdgeFast(other) {
        for (let i = 0; i < this.edges.length; i++) {
            if (this.edges[i] === other) return true
        }
        return false;
    }

    hasEdge(other) {
        return this.hasEdgeFast(other)
    }

}

class DataNode extends GraphNode {
    constructor(id, data) {
        super()
        this.data = data
        this.id = id
        this.size = 1
    }
}

class DataBuild {

    springLength = 500; //弹簧静止长度
    springStrength = 0.0001; //弹性系数
    repulsionStrength = 1500; //库伦常数
    skipDistance = 575; // 距离过远跳过计算
    maxSecond = 20000; // 距离过远跳过计算
    step = 1;

    builds(datas, links) {
        let data_nodes = []
        let m_data = {}
        datas.forEach(data => {
            let n = new DataNode(data.id, data)
            m_data[data.id] = n
            data_nodes.push(n)
        });
        links.forEach(link => {
            m_data[link.source].connect(m_data[link.target])
            m_data[link.target].connect(m_data[link.source])
            m_data[link.source].size += this.step
            m_data[link.target].size += this.step
        });
        for (let i = 0; i < this.maxSecond; i++) {
            this.forceDirected_noRepeat_skip_noVector(data_nodes)
        }
        return data_nodes
    }

    forceDirected_noRepeat_skip_noVector(graph) {
        for (let i = 0; i < graph.length; i++) {
            let node = graph[i];
            for (let j = i + 1; j < graph.length; j++) {
                let other = graph[j];

                let apartX = other.pos.x - node.pos.x;
                let apartY = other.pos.y - node.pos.y;
                let distance = Math.max(1, Math.sqrt(apartX * apartX + apartY * apartY))

                let hasEdge = node.hasEdge(other);
                if (!hasEdge && distance > this.skipDistance) continue;
                let forceSize = -this.repulsionStrength / (distance * distance);
                if (node.hasEdge(other)) {
                    forceSize += (distance - this.springLength) * this.springStrength;
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
}
