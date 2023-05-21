import React from "react"

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

const OrderTable: React.FC<{ orders: Order[]; isBid?: boolean }> = ({
    orders,
    isBid,
}) => {
    const validOrders = orders.filter(
        (order) => order.price !== 0 && order.quantity !== 0
    )

    return (
        <table className="table-fixed max-h-12 mx-4 mt-2">
            <thead>
                <tr>
                    <th className="px-2">{isBid ? "Qty" : "Price"}</th>
                    <th className="px-2">{isBid ? "Price" : "Qty"}</th>
                </tr>
            </thead>
            <tbody>
                {validOrders.map((order) => (
                    <tr key={order.id} className="border-y">
                        <td className={isBid ? "" : "text-red-400"}>
                            {isBid ? order.quantity : order.price.toFixed(2)}
                        </td>
                        <td className={isBid ? "text-green-400" : ""}>
                            {isBid ? order.price.toFixed(2) : order.quantity}
                        </td>
                    </tr>
                ))}
            </tbody>
        </table>
    )
}

const Book: React.FC<Props> = ({ book }) => {
    if (!book) {
        return <div>Loading...</div>
    }

    return (
        <div className="p-4 text-center w-[370px] h-[450px] border">
            <div className="border-b">Order Book</div>
            <div className="flex">
                <OrderTable orders={book.bids} isBid={true} />
                <OrderTable orders={book.asks} />
            </div>
        </div>
    )
}

export default Book
