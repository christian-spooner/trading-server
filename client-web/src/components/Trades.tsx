import React from "react"

interface Props {
    trades: Array<[number, number, string]>
}

const formatTradeTime = (timestamp: string) => {
    let time = timestamp.split("T")[1].split(".")[0]
    if (time.endsWith("Z")) {
        time = time.slice(0, -1)
    }
    return time
}

const Trades: React.FC<Props> = ({ trades }) => {
    return (
        <div className="p-4 text-center w-[370px] border h-[450px]">
            <div className="border-b">Market Trades</div>
            <table className="table-fixed max-h-12 mx-2 mt-2">
                <thead className="">
                    <tr className="">
                        <th className="px-2">Qty</th>
                        <th className="px-4">Price</th>
                        <th className="px-6">Time</th>
                    </tr>
                </thead>
                <tbody>
                    {trades
                        .filter((t: [number, number, string]) => t[0] !== 0)
                        .map((trade: [number, number, string], index) => {
                            const nextQuantity =
                                index === trades.length - 1
                                    ? trades[index - 1][1]
                                    : trades[index + 1][1]
                            return (
                                <tr key={trade[2]} className="border-y">
                                    <td>{trade[0]}</td>
                                    <td
                                        className={
                                            trade[1] >= nextQuantity
                                                ? "text-green-400"
                                                : "text-red-400"
                                        }
                                    >
                                        {trade[1].toFixed(2)}
                                    </td>
                                    <td>{formatTradeTime(trade[2])}</td>
                                </tr>
                            )
                        })}
                </tbody>
            </table>
        </div>
    )
}

export default Trades
