import time


class TransactionLedger:
    def __init__(self):
        self._history = []
        self._volume = 0

    """
    Public Methods
    """

    def append_txn(self, txn):
        txn["timestamp"] = time.time()
        self._history.append(txn)
        self._volume += 1

    def get_txns(self):
        return self._history

    def get_total_volume(self):
        return self._volume

    def get_volume_per_second(self):
        current_time = time.time()
        vps = 0
        for txn in reversed(self._history):
            if txn["timestamp"] < current_time - 1:
                break
            vps += 1
        return vps
