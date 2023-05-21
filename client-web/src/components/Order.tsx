import React, { useState } from "react"

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
    updatePositions: (positions: Position[]) => void
}

interface OrderValues {
    quantity: string
    price: string
}

const Order: React.FC<Props> = ({ positions, updatePositions }) => {
    const [tab, setTab] = useState("Buy")
    const [orderValues, setOrderValues] = useState<OrderValues>({
        quantity: "",
        price: "",
    })

    const handleTabClick = (tab: string) => {
        setTab(tab)
    }

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setOrderValues({
            ...orderValues,
            [e.target.name]: e.target.value,
        })
    }

    const sendOrder = async (order: {
        side: string
        quantity: number
        price: number
    }) => {
        const response = await fetch("http://localhost:3000/order", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(order),
        })
        return await response.json()
    }

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault()
        const quantity = parseInt(orderValues.quantity)
        const price = parseFloat(orderValues.price)
        if (isNaN(quantity) || isNaN(price)) {
            // Display an error message or set an error state
            alert("Invalid values")
            return
        }
        try {
            const orderRequest = {
                side: tab,
                quantity: quantity,
                price: price,
            }
            const data = await sendOrder(orderRequest)
            alert(`Order submitted successfully, ID: ${data.id}`)
            var currentUTCTime = new Date()
            var formattedTime = `${currentUTCTime.getUTCHours()}:${currentUTCTime.getUTCMinutes()}`
            const newPosition: Position = {
                id: data.id,
                side: orderRequest.side,
                price: orderRequest.price,
                quantity: orderRequest.quantity,
                time: formattedTime,
                status: "New",
            }
            updatePositions([...positions, newPosition])
            setOrderValues({
                quantity: "",
                price: "",
            })
        } catch (err) {
            console.error(err)
        }
    }

    return (
        <div className="p-4 text-center w-[370px] border mt-8 h-[450px]">
            <div className="flex justify-center border-b">
                <div className={tab === "Buy" ? "bg-sky-500/10 rounded" : ""}>
                    <button
                        className="w-40"
                        onClick={() => handleTabClick("Buy")}
                    >
                        Buy
                    </button>
                </div>
                <div className={tab === "Sell" ? "bg-sky-500/10 rounded" : ""}>
                    <button
                        className="w-40"
                        onClick={() => handleTabClick("Sell")}
                    >
                        Sell
                    </button>
                </div>
            </div>
            <form onSubmit={handleSubmit} className="mt-2" autoComplete="off">
                <div>
                    <div className="flex justify-center bg-sky-500/10 rounded-md mx-4 my-6 h-10 py-1.5">
                        <div className="w-[48px] text-gray-600">Qty</div>
                        <input
                            type="text"
                            name="quantity"
                            className="bg-transparent text-right focus:outline-none px-2 w-[200px]"
                            value={orderValues.quantity}
                            onChange={handleInputChange}
                        />
                        <div className="w-[48px]"></div>
                    </div>
                    <div className="flex justify-center bg-sky-500/10 rounded-md mx-4 my-6 h-10 py-1.5">
                        <div className="w-[48px] text-gray-600">Price</div>
                        <input
                            type="text"
                            name="price"
                            className="bg-transparent text-right focus:outline-none px-2 w-[200px]"
                            value={orderValues.price}
                            onChange={handleInputChange}
                        />
                        <div className="w-[48px]">USD</div>
                    </div>
                </div>
                <button
                    type="submit"
                    className="px-4 py-1 rounded-md hover:border hover:py-[3px]"
                >
                    Submit
                </button>
            </form>
        </div>
    )
}

export default Order
