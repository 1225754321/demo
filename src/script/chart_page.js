let chart_page = {
    "type": "page",
    "body": {
        "type": "chart",
        "api": "/amis/api/mock2/chart/chartData",
        "config": {
            tooltip: {},
            legend: [
                {
                    data: "${ARRAYMAP(categories, item => item.name)}"
                }
            ],
            series: [
                {
                    name: 'Les Miserables',
                    type: 'graph',
                    layout: 'none',
                    data: "${nodes}",
                    links: "${links}",
                    categories: "${categories}",
                    roam: true,
                    label: {
                        show: true,
                        position: 'right',
                        formatter: '{b}',
                        color: "inherit"
                    },
                    labelLayout: {
                        hideOverlap: true
                    },
                    scaleLimit: {
                        min: 0.4,
                        max: 2
                    },
                    lineStyle: {
                        color: 'source',
                        curveness: 0.3
                    },
                }
            ]
        }
    }
};