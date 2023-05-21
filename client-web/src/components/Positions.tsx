import React from "react"

interface Position {
    id: number
    side: string
    price: number
    quantity: number
    time: string
    status: string
}

interface Props {
    positions: Array<Position>
}

const getColorClass = (
    condition: boolean,
    trueClass: string,
    falseClass: string
) => (condition ? trueClass : falseClass)

const Positions: React.FC<Props> = ({ positions }) => {
    const recentPositions = positions.slice(-12)

    return (
        <div className="p-4 text-center w-auto h-[450px] border mt-8">
            <div className="border-b">Positions</div>
            <table className="table-fixed max-h-12 mx-4 mt-2">
                <thead>
                    <tr>
                        <th className="px-2">Side</th>
                        <th className="px-2">Qty</th>
                        <th className="px-2">Price</th>
                        <th className="px-4">Sent</th>
                        <th className="px-2">Status</th>
                    </tr>
                </thead>
                <tbody>
                    {recentPositions.map(
                        ({ id, side, quantity, price, time, status }) => (
                            <tr key={id} className="border-y">
                                <td
                                    className={getColorClass(
                                        side === "Buy",
                                        "text-green-400",
                                        "text-red-400"
                                    )}
                                >
                                    {side}
                                </td>
                                <td>{quantity}</td>
                                <td
                                    className={getColorClass(
                                        side === "Buy",
                                        "text-green-400",
                                        "text-red-400"
                                    )}
                                >
                                    {price.toFixed(2)}
                                </td>
                                <td>{time}</td>
                                <td
                                    className={getColorClass(
                                        status === "New",
                                        "text-white",
                                        "text-green-400"
                                    )}
                                >
                                    {status}
                                </td>
                            </tr>
                        )
                    )}
                </tbody>
            </table>
        </div>
    )
}

export default Positions
