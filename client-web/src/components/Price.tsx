import React, { useEffect, useState, useRef } from "react"
import { init, dispose, Chart } from "klinecharts"

interface PriceDatum {
    open: number
    close: number
    high: number
    low: number
    timestamp: number
}

interface Props {
    price: PriceDatum
}

const Price: React.FC<Props> = ({ price }) => {
    const [priceData, setPriceData] = useState<PriceDatum[]>([])
    const chartRef = useRef<Chart | null>(null)

    useEffect(() => {
        if (price && price.timestamp !== 0) {
            setPriceData((prevPriceData) => [...prevPriceData, price])
        }
    }, [price])

    useEffect(() => {
        chartRef.current = init("simple_chart")
        chartRef.current?.createIndicator(
            {
                name: "MA",
                calcParams: [7, 25],
            },
            false,
            {
                id: "candle_pane",
            }
        )

        chartRef.current?.applyNewData(priceData)

        return () => dispose("simple_chart")
    }, [priceData])

    return (
        <div className="p-4 text-center w-[800px] h-[450px] border">
            <div className="border-b">Price</div>
            <div id="simple_chart" style={{ height: 400 }} />
        </div>
    )
}

export default Price
