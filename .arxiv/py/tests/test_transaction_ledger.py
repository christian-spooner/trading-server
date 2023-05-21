from src.transaction_ledger import TransactionLedger

txn_1 = {
    "buyer_id": 1,
    "seller_id": 2,
    "price": 10,
    "quantity": 1,
}
txn_2 = {
    "buyer_id": 2,
    "seller_id": 1,
    "price": 10,
    "quantity": 1,
}


def test_get_append_txns():
    transactionLedger = TransactionLedger()
    transactionLedger.append_txn(txn_1)
    transactionLedger.append_txn(txn_2)
    assert len(transactionLedger.get_txns()) == 2
