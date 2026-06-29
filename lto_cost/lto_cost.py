from dataclasses import dataclass
from math import ceil


# USD price of one tape drive
LTO_DRIVE_PRICES = {
    "lto-10": 13000.0,
    "lto-9": 4500.0,
    "lto-8": 2500.0,
    "lto-7": 650.0,
    "lto-6": 425.0,
}


@dataclass(frozen=True)
class Tape:
    name: str
    capacity_tb: float
    tape_price_usd: float
    drive_key: str

    @property
    def media_price_per_tb(self) -> float:
        return self.tape_price_usd / self.capacity_tb

    def total_cost(self, required_tb: float) -> float:
        tapes_needed = ceil(required_tb / self.capacity_tb)
        return (
            LTO_DRIVE_PRICES[self.drive_key]
            + tapes_needed * self.tape_price_usd
        )

    def effective_price_per_tb(self, required_tb: float) -> float:
        return self.total_cost(required_tb) / required_tb


TAPES = [
    Tape("LTO-10 40TB", 40, 500, "lto-10"),
    Tape("LTO-10 30TB", 30, 287, "lto-10"),
    Tape("LTO-9 18TB", 18, 99, "lto-9"),
    Tape("LTO-8 12TB", 12, 65, "lto-8"),
    Tape("LTO-7 6TB", 6, 58, "lto-7"),
    Tape("LTO-6 2.5TB", 2.5, 30.25, "lto-6"),
]


def rank_by_storage(required_tb: float):
    return sorted(
        TAPES,
        key=lambda tape: tape.effective_price_per_tb(required_tb),
    )


def print_rankings(required_tb: float):
    print(f"Storage required: {required_tb:.1f} TB\n")

    print(
        f"{'Tape':20} {'Drive':8} {'Tapes':>5} "
        f"{'Total $':>10} {'$/TB':>8}"
    )
    print("-" * 60)

    for tape in rank_by_storage(required_tb):
        tapes_needed = ceil(required_tb / tape.capacity_tb)
        total = tape.total_cost(required_tb)

        print(
            f"{tape.name:20}"
            f"{tape.drive_key:8}"
            f"{tapes_needed:5}"
            f"{total:10.2f}"
            f"{total / required_tb:8.2f}"
        )


if __name__ == "__main__":
    required_tb = 500
    print_rankings(required_tb)
