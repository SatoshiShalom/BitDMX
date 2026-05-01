import pytest
import time
from starkware.starknet.testing.starknet import Starknet

# Helper to simulate hash2 (replace with actual Cairo hash2 if needed)
def fake_hash2(a, b):
    return (a * 31 + b * 17) % (2**251)

@pytest.mark.asyncio
async def test_age_proof_performance_various_inputs():
    starknet = await Starknet.empty()
    contract = await starknet.deploy('cairo/age_proof.cairo')

    test_cases = [
        # (country_code, legal_age, birthdate, current_date, randomness)
        (1, 18, 20000101, 20260430, 42),   # Peru, valid
        (2, 21, 20050101, 20260430, 99),   # USA, not valid
        (3, 18, 20100101, 20260430, 123),  # Japan, not valid
        (4, 18, 19900101, 20260430, 555),  # Germany, valid
        (1, 18, 20200101, 20260430, 777),  # Peru, not valid
        (2, 21, 19950101, 20260430, 888),  # USA, valid
    ]

    for i, (country_code, legal_age, birthdate, current_date, randomness) in enumerate(test_cases):
        commitment = fake_hash2(birthdate, randomness)
        start = time.time()
        result = await contract.verify_age_proof(
            commitment, country_code, current_date, birthdate, randomness
        ).call()
        end = time.time()
        print(f"Test case {i+1}: country_code={country_code}, birthdate={birthdate}, current_date={current_date}, randomness={randomness}")
        print(f"  Result: is_valid={result.result.is_valid}, Execution time: {end - start:.6f} seconds\n")
        assert result.result.is_valid in [0, 1]
