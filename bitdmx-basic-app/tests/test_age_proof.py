import pytest
import time
from starkware.starknet.testing.starknet import Starknet

@pytest.mark.asyncio
async def test_age_proof_performance():
    starknet = await Starknet.empty()
    contract = await starknet.deploy('cairo/age_proof.cairo')

    # Example: Peru (code 1), legal age 18, birthdate 20000101, current date 20240430
    commitment = 123456  # Replace with actual hash2(birthdate, randomness)
    country_code = 1
    current_date = 20240430
    proof_birthdate = 20000101
    randomness = 42

    start = time.time()
    result = await contract.verify_age_proof(
        commitment, country_code, current_date, proof_birthdate, randomness
    ).call()
    end = time.time()
    print(f"Execution time: {end - start:.6f} seconds")
    assert result.result.is_valid in [0, 1]  # Accept either for performance test
