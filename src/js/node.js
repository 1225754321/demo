class Dot {
    constructor(id, data, size, x, y) {
        this.id = id
        this.data = data
        this.size = size
        this.x = x
        this.y = y
        this.is_fixed = false
    }
}

class Line {
    constructor(source, target) {
        this.source = source
        this.target = target
    }
}

function start(dots, lines, height = 1920, width = 1080) {
    let m_dots = {};
    let sources = {};
    let targets = {};
    dots.forEach(dot => {
        m_dots[dot.id] = dot
    });
    lines.forEach(line => {
        if (!sources[line.source]) {
            sources[line.source] = []
        }
        if (!targets[line.target]) {
            targets[line.target] = []
        }
        sources[line.source].push(m_dots[line.target])
        targets[line.target].push(m_dots[line.source])
        m_dots[line.source].size += 1
        m_dots[line.target].size += 1
    });
    dots.sort((a, b) => a.size - b.size)
    // 最小圆半径
    let br = 4; // 每多一个连接半径+0.1
}