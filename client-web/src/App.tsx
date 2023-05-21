import { useState, useEffect } from "react"
import Book from "./components/Book"
import Price from "./components/Price"
import Order from "./components/Order"
import Trades from "./components/Trades"
import Depth from "./components/Depth"
import Positions from "./components/Positions"

interface PriceDatum {
    open: number
    close: number
    high: number
    low: number
    timestamp: number
}

const useFetch = (
    url: string,
    interval: number,
    setState: React.Dispatch<React.SetStateAction<any>>
) => {
    useEffect(() => {
        const fetchData = async () => {
            const response = await fetch(url)
            const data = await response.json()
            setState(data)
        }

        fetchData()

        const intervalId = setInterval(fetchData, interval)

        return () => clearInterval(intervalId)
    }, [url, interval, setState])
}

function App() {
    const [book, setBook] = useState({ bids: [], asks: [] })
    const [price, setPrice] = useState<PriceDatum>({
        open: 0,
        close: 0,
        high: 0,
        low: 0,
        timestamp: 0,
    })
    const [trades, setTrades] = useState([])
    const [positions, setPositions] = useState(() => {
        const storedPositions = localStorage.getItem("positions")
        return storedPositions ? JSON.parse(storedPositions) : []
    })

    // Fetch data
    useFetch("http://localhost:3000/book", 5000, setBook)
    useFetch("http://localhost:3000/price", 5000, setPrice)
    useFetch("http://localhost:3000/trades", 5000, setTrades)

    // Update positions
    useEffect(() => {
        const intervalId = setInterval(() => {
            positions.forEach((position: { id: any }) => {
                fetch(`http://localhost:3000/report/${position.id}`)
                    .then((res) => res.json())
                    .then((data) => {
                        if (data.hasOwnProperty("status")) {
                            const status = data.status.split(" ")[0]
                            setPositions((prevPositions: any[]) => {
                                const updatedPosition = {
                                    ...position,
                                    status: status,
                                }
                                const updatedPositions = prevPositions.map(
                                    (p) =>
                                        p.id === updatedPosition.id
                                            ? updatedPosition
                                            : p
                                )
                                return updatedPositions
                            })
                        }
                    })
            })
        }, 10000)

        return () => {
            clearInterval(intervalId)
        }
    }, [positions])

    return (
        <div className="flex space-x-8 p-6">
            <div>
                <Price price={price} />
                <Depth book={book} />
            </div>
            <div>
                <Book book={book} />
                <Positions positions={positions} />
            </div>
            <div>
                <Trades trades={trades} />
                <Order positions={positions} updatePositions={setPositions} />
            </div>
        </div>
    )
}

export default App
