Trading Mechanics
================================================================================

What is a trade?

>A trade is an agreement between two or more parties to exchange financial instruments. Trades are typically executed through an electronic trading system that facilitates the matching of buy and sell orders.

What is electronic trading?

>Electronic trading refers to the use of computer systems and software to facilitate the trading of financial instruments such as stocks, bonds, and derivatives.

What is an electonic trading system?

>An electronic trading system is a computer program or network of computers that facilitates the buying and selling of financial instruments, such as stocks, bonds, and derivatives. These systems can be used for exchange-traded securities as well as over-the-counter (OTC) markets. They typically include advanced features such as real-time quotes, charting tools, and advanced order types. Electronic trading systems can be used by individual traders, as well as by large financial institutions.

Components of an electronic trading system:

> - Market data feed handlers
> - Historical data stores
> - Order manangement system
> - Signal aggregators
> - Trade analytics
> - Risk management system
> - Execution logic

What is an electronic exchange?

>An electronic exchange is a marketplace that uses electronic trading systems to facilitate the buying and selling of financial instruments, such as stocks, bonds, and derivatives. These systems can be used for exchange-traded securities as well as OTC markets. Electronic exchanges are typically organized and regulated, and may be owned by a group of market participants or by a government. They provide a platform for traders to buy and sell securities through a centralized market place and also provide transparency, price discovery and market surveillance to the market participants. Some examples of electronic exchanges include the New York Stock Exchange (NYSE), the Nasdaq, and the Chicago Mercantile Exchange (CME).

Types of electronic exchanges:

> - Centralized exchanges
> - Decentralized exchanges
> - Dark pools
> - Electronic communication networks
> - Hybrid exchanges

Components of an electronic exchange:

> - Order book
> - Matching engine
> - Clearing and settlement system
> - Market data feed handlers

What is an order book?

>An order book is a digital record of buy and sell orders for a particular financial instrument, such as a stock, bond, or derivative. It is a fundamental component of an electronic trading system and it is used to match buy and sell orders. The order book shows the prices and quantities of all outstanding buy and sell orders at a given moment.

>The order book can be divided into two parts, the bid side and the ask side. The bid side shows the highest price at which a buyer is willing to purchase the security and the quantity they are willing to buy at that price. The ask side shows the lowest price at which a seller is willing to sell the security and the quantity they are willing to sell at that price. The difference between the highest bid and the lowest ask is called the bid-ask spread.

>The order book is constantly updating as new orders are placed, cancelled or executed. It is used to determine the current market price of a security by the highest bid and lowest ask prices. It also provides transparency and price discovery to the market participants. The order book is used by a matching engine to match orders and execute trades.

What is a matching engine?

>A matching engine is a software component of an electronic trading system that is responsible for matching buy and sell orders for financial instruments, such as stocks, bonds, and derivatives. The matching engine typically uses complex algorithms to determine the best price and execution conditions for a given trade based on the available orders in the system.

>Matching engines are used in a variety of electronic trading systems, including electronic exchanges, dark pools, and ECNs. They are designed to match buy and sell orders as quickly and efficiently as possible, taking into account factors such as price, time, and volume.

>Matching engines can be implemented in a variety of ways, depending on the specific requirements of the system. Some use first-in, first-out (FIFO) algorithms, while others use price-time priority or pro-rata algorithms.

Matching algorithms:

> - Price-time priority: buy order + sell order with lowest price and earliest time
> - Time priority: buy order + sell order with earliest time
> - Pro-rata: buy order + sell orders in proportion to their volume (at the same price)
> - Volume priority: buy order + sell order with greatest volume
> - Price-size priority: buy order + sell order with lowest price and greatest volume

What is an order management system?

>An order management system (OMS) is a software system used to automate and manage the various steps involved in executing a trade, such as routing orders to the appropriate trading venue, managing risk, and allocating trades to the appropriate accounts. OMSs are often used by institutional traders and investment managers to streamline their trading operations and improve efficiency.

What is a clearing and settlement system?

>A clearing and settlement system is a process used to facilitate the transfer of ownership of financial instruments between buyers and sellers. Clearing refers to the process of matching buy and sell orders, ensuring that all the necessary information is correct and that the parties involved have the necessary funds or securities to complete the transaction. Settlement refers to the process of transferring the ownership of the financial instrument and the associated funds between the parties involved.

Order types:

> - Market order
> - Limit order
> - Stop order
> - Stop-limit order
> - Trailing stop order
> - Fill-or-kill order
> - All-or-none order
> - Good-till-cancelled order
> - Good-till-date order
> - Day order

Execution mechanisms:

> - Market execution
> - Limit execution
> - Pegged execution
> - Auction execution
> - Crossing execution

What is the difference between a market order and a limit order?

>A market order is an order to buy or sell a financial instrument at the best available price. A limit order is an order to buy or sell a financial instrument at a specified price or better.

What is OTC trading?
 
>OTC (over-the-counter) trading refers to the buying and selling of financial instruments directly between two parties, rather than through a formal exchange. This type of trading is typically used for financial instruments such as derivatives, which are customized to meet the specific needs of the parties involved, and for which there is not a formal exchange market.

>OTC trading is generally considered to be less regulated and less transparent than trading on a formal exchange. However, it also offers more flexibility in terms of the types of contracts that can be traded, and the terms and conditions of those contracts.

>OTC markets are not standardized, and the prices and conditions of a transaction are negotiated directly between the parties involved. This means that OTC transactions are often considered to be higher risk and are typically only undertaken by experienced, well-capitalized traders and institutions.

What is a dark pool?

>A dark pool is a type of private financial forum or platform that allows traders to buy and sell large blocks of securities without revealing the details of the trade to the public. Dark pools are typically operated by large financial institutions and are designed to provide anonymity to traders, allowing them to execute large trades without affecting the market price of the security.

>Dark pools are also known as "dark liquidity pools" or "alternative trading systems" (ATS). Because the trades are not reported publicly, it is difficult to know the exact volume of trading that takes place in dark pools, but it is estimated that dark pools account for a significant portion of overall trading volume.

>While dark pools can provide benefits to traders such as anonymity and reduced market impact, there are also concerns that they may be used to engage in insider trading or other forms of market manipulation. Regulators in some countries have imposed stricter rules on the operation of dark pools in recent years to address these concerns.

What is an electronic communication network?

>An electronic communication network (ECN) is a type of electronic trading system that facilitates the trading of financial instruments, such as stocks, bonds, and derivatives. ECNs are typically used for exchange-traded securities as well as OTC markets. They provide a platform for traders to buy and sell securities through a decentralized market place, matching buy and sell orders electronically.

>ECNs differ from traditional exchanges in that they are not a central marketplace or exchange but rather a network of market participants that connect to one another to trade securities. They allow traders to access a wide range of liquidity providers, including other traders, market makers, and institutional investors. This results in more competitive prices and greater transparency for traders.

>ECNs often provide direct market access (DMA) to traders, which means that traders can send their orders directly to the market without going through a broker or dealer. This can reduce the cost and speed of trading. Some examples of ECN's include Instinet and Island ECN.

What is RFQ?

>RFQ (request for quote) is a process in which a potential buyer or seller of a financial instrument sends a request to multiple counterparties for a quote on the price at which they are willing to buy or sell the instrument.

>The RFQ process allows the buyer or seller to compare quotes from multiple counterparties and select the most favorable one. This process is typically used for financial instruments such as derivatives, which are customized to meet the specific needs of the parties involved, and for which there is not a formal exchange market.

>RFQ systems can be automated, with the RFQ process being executed through an electronic platform, or it can be done manually, with the buyer or seller contacting potential counterparties directly.

>RFQ is not a binding agreement, it only serves as a mechanism for obtaining quotes from multiple potential counterparties, which can help the buyer or seller to make an informed decision.

Market participants:

> - Market makers
> - Liquidity providers
> - Market takers
> - Arbitrageurs
> - Speculators
> - Hedgers

What is a liquidity provider?

>A liquidity provider is a market participant that provides liquidity to the market by placing buy and sell orders. Liquidity providers include market makers, brokers, and other market participants.

What is a market maker?

>A market maker is a market participant that provides liquidity to the market by placing buy and sell orders. Market makers typically make a profit by taking the bid-ask spread, which is the difference between the best bid and best ask prices.

What is a broker?

>A broker is a market participant that provides brokerage services to traders and investors. Brokers typically charge a commission for executing trades on behalf of their clients.

What happens to an order placed on a brokerage app?

>When a trader places an order on a brokerage app, the order is sent to the brokerage's trading server. The trading server then routes the order to the appropriate exchange or liquidity provider. The exchange or liquidity provider then sends the order to the matching engine, which matches the order with other orders in the order book. The matching engine then sends the trade to the clearing and settlement system, which settles the trade and updates the order book.

What is market microstructure?

>Market microstructure refers to the rules and practices that govern the operation of financial markets and the interaction between market participants. It includes the mechanisms by which orders are matched, the sources of liquidity, and the transparency of the market.

>Market microstructure is an important consideration for traders and investors because it can affect the cost and speed of executing trades, as well as the level of competition and liquidity in the market. It can also impact the quality of price discovery, which is the process by which prices for assets are determined.

What is a tick?

>A tick is the smallest price change in a financial instrument. For example, a stock with a tick size of $0.01 can only change in price by $0.01 increments.

What is the bid-ask spread?

>The bid-ask spread is the difference between the best bid (buy) and best ask (sell) prices for a financial instrument. It is typically expressed as a percentage of the last traded price.

What is FIX?

>FIX is an open standard for electronic trading that is used by brokers, exchanges, and other market participants to facilitate the trading of financial instruments. It is maintained by the FIX Protocol Limited, a non-profit organization that is owned by its members.

What is high-freqency trading (HFT)?

>HFT is a type of algorithmic trading that uses advanced technology and high-speed networks to execute a high volume of trades at very fast speeds. HFT systems use complex algorithms to analyze market data and make trades in milliseconds.

What is a risk engine?

>A risk engine is a component of an electronic trading system that is responsible for monitoring and managing the risks associated with trading. It typically uses complex algorithms to assess the potential risks of trades and make decisions about whether to execute them.

What is smart order routing?

>Smart order routing is the use of algorithms to route orders to the best available liquidity source. It can include routing orders to different exchanges, ECNs, and dark pools based on factors such as price, volume, and execution speed.

What is colocation?

>Colocation refers to the practice of placing a trader's computer servers in the same physical location as the exchange's servers to reduce latency and improve the speed of trade execution. By having the servers in the same location, the data can be transferred faster, resulting in faster trade execution.

What is trade latency?

>Trade latency refers to the time it takes for a trade to be executed, from the moment an order is placed to the moment it is matched and completed. Latency is an important consideration in electronic trading, as faster trade execution can result in better prices and more favorable trading conditions.

What is direct market access?

>Direct Market Access (DMA) is a service that allows traders to send their orders directly to the market, bypassing a broker or dealer. This can reduce the cost and speed of trading, as well as improve the transparency and control of trades.
