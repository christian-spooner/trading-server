import { useEffect, useState, FC } from "react"
import Highcharts from "highcharts"
import HighchartsReact from "highcharts-react-official"
import isEqual from "lodash/isEqual"

interface Order {
    id: number
    side: string
    quantity: number
    price: number
}

interface Book {
    bids: Order[]
    asks: Order[]
}

interface Props {
    book: Book
}

const Depth: FC<Props> = ({ book }) => {
    const [chartRef, setChartRef] = useState<HighchartsReact.RefObject | null>(
        null
    )
    const [chartOptions, setChartOptions] = useState({})

    useEffect(() => {
        // Intermediate step: grouping by price
        const bidGrouped = book.bids.reduce((acc, order) => {
            if (!acc[order.price]) {
                acc[order.price] = 0
            }
            acc[order.price] += order.quantity
            return acc
        }, {} as Record<number, number>)

        // Calculate cumulative quantities
        const bidDepth = Object.entries(bidGrouped)
            .sort(
                ([aPrice], [bPrice]) => parseFloat(bPrice) - parseFloat(aPrice)
            ) // reverse order for bids
            .reduce((acc, [price, quantity]) => {
                let cumulativeQuantity = quantity
                if (acc.length > 0) {
                    cumulativeQuantity += acc[acc.length - 1][1]
                }
                acc.push([parseFloat(price), cumulativeQuantity])
                return acc
            }, [] as [number, number][])

        // Intermediate step: grouping by price
        const askGrouped = book.asks.reduce((acc, order) => {
            if (!acc[order.price]) {
                acc[order.price] = 0
            }
            acc[order.price] += order.quantity
            return acc
        }, {} as Record<number, number>)

        // Calculate cumulative quantities
        const askDepth = Object.entries(askGrouped)
            .sort(
                ([aPrice], [bPrice]) => parseFloat(aPrice) - parseFloat(bPrice)
            )
            .reduce((acc, [price, quantity]) => {
                let cumulativeQuantity = quantity
                if (acc.length > 0) {
                    cumulativeQuantity += acc[acc.length - 1][1]
                }
                acc.push([parseFloat(price), cumulativeQuantity])
                return acc
            }, [] as [number, number][])

        console.log(bidDepth)

        if (
            chartRef &&
            chartRef.chart &&
            chartRef.chart.series &&
            chartRef.chart.series.length > 1 &&
            !(
                isEqual(bidDepth, chartRef.chart.series[0].data) &&
                isEqual(askDepth, chartRef.chart.series[1].data)
            )
        ) {
            const chart = chartRef.chart
            chart.series[0].setData(bidDepth, true)
            chart.series[1].setData(askDepth, true)
        } else {
            setChartOptions({
                accessibility: {
                    enabled: false,
                },
                chart: {
                    type: "area",
                    zoomType: "xy",
                    backgroundColor: null,
                },
                title: {
                    text: "",
                },
                xAxis: {
                    minPadding: 0,
                    maxPadding: 0,
                    plotLines: [
                        {
                            color: "#888",
                            value: 0.1523,
                            width: 1,
                            label: {
                                text: "",
                                rotation: 90,
                            },
                        },
                    ],
                    title: {
                        text: "",
                    },
                },
                yAxis: [
                    {
                        lineWidth: 1,
                        gridLineWidth: 1,
                        title: null,
                        tickWidth: 1,
                        tickLength: 5,
                        tickPosition: "inside",
                        labels: {
                            align: "left",
                            x: 8,
                        },
                    },
                    {
                        opposite: true,
                        linkedTo: 0,
                        lineWidth: 1,
                        gridLineWidth: 0,
                        title: null,
                        tickWidth: 1,
                        tickLength: 5,
                        tickPosition: "inside",
                        labels: {
                            align: "right",
                            x: -8,
                        },
                    },
                ],
                legend: {
                    enabled: false,
                },
                plotOptions: {
                    area: {
                        fillOpacity: 0.2,
                        lineWidth: 1,
                    },
                },
                tooltip: {
                    backgroundColor: null,
                    borderWidth: 0,
                    style: {
                        color: "white",
                    },
                },
                credits: {
                    enabled: false,
                },
                series: [
                    {
                        name: "Bids",
                        data: bidDepth,
                        color: "#03a7a8",
                        step: "right",
                    },
                    {
                        name: "Asks",
                        data: askDepth,
                        color: "#fc5857",
                        step: "left",
                    },
                ],
            })
        }
    }, [book, chartRef])

    const afterChartCreated = (chartRef: HighchartsReact.RefObject) => {
        setChartRef(chartRef)
    }

    return (
        <div className="p-4 text-center w-auto border h-[450px] mt-8">
            <div className="border-b">Depth</div>
            <HighchartsReact
                highcharts={Highcharts}
                options={chartOptions}
                callback={afterChartCreated}
            />
        </div>
    )
}

export default Depth
