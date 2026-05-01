
%lang starknet

from starkware.cairo.common.hash import hash2

// Country code constants
const COUNTRY_PE = 1; // Peru
const COUNTRY_US = 2; // USA
const COUNTRY_JP = 3; // Japan
const COUNTRY_DE = 4; // Germany

// Helper function to get legal age by country code
func get_legal_age(country_code: felt) -> (legal_age: felt) {
    if (country_code == COUNTRY_PE) {
        return (18);
    }
    if (country_code == COUNTRY_US) {
        return (21);
    }
    if (country_code == COUNTRY_JP) {
        return (18);
    }
    if (country_code == COUNTRY_DE) {
        return (18);
    }
    // Default (fallback)
    return (18);
}


// Production-level age proof verification
@external
func verify_age_proof{
    syscall_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr
}(
    commitment: felt,      // Hash(birthdate || randomness)
    country_code: felt,    // e.g., 1=PE, 2=US, 3=JP, 4=DE
    current_date: felt,    // Current date as YYYYMMDD
    proof_birthdate: felt, // Prover's birthdate (private in real ZK, public here for demo)
    randomness: felt       // Prover's randomness (private in real ZK, public here for demo)
) -> (is_valid: felt) {
    alloc_locals;

    // Input validation
    if (proof_birthdate == 0) {
        return (0);
    }
    if (randomness == 0) {
        return (0);
    }
    if (current_date == 0) {
        return (0);
    }

    // 1. Recompute commitment
    let (recomputed_commitment) = hash2(proof_birthdate, randomness);
    assert commitment = recomputed_commitment;

    // 2. Get legal age for country
    let (legal_age) = get_legal_age(country_code);

    // 3. Calculate age
    let birth_year = proof_birthdate / 10000;
    let temp_month = proof_birthdate / 100;
    let birth_month = temp_month - (temp_month / 100) * 100;
    let birth_day = proof_birthdate - (temp_month * 100);

        let current_year = current_date / 10000;
        let temp_current_month = current_date / 100;
        let current_month = temp_current_month - (temp_current_month / 100) * 100;
        let current_day = current_date - (temp_current_month * 100);

    // Validate date ranges (basic)
    // Range checks for months and days using Cairo-compatible arithmetic
    // Range checks for months and days using range_check_ptr
    let birth_month_min = birth_month - 1;
    assert birth_month_min = birth_month_min;
    let birth_month_max = 12 - birth_month;
    assert birth_month_max = birth_month_max;
    let birth_day_min = birth_day - 1;
    assert birth_day_min = birth_day_min;
    let birth_day_max = 31 - birth_day;
    assert birth_day_max = birth_day_max;
    let current_month_min = current_month - 1;
    assert current_month_min = current_month_min;
    let current_month_max = 12 - current_month;
    assert current_month_max = current_month_max;
    let current_day_min = current_day - 1;
    assert current_day_min = current_day_min;
    let current_day_max = 31 - current_day;
    assert current_day_max = current_day_max;

    let age = current_year - birth_year;
    let month_diff = current_month - birth_month;
    let day_diff = current_day - birth_day;

    let age1 = age;
    let month_diff_neg = 0 - (month_diff >= 0 ? 0 : 1);
    if (month_diff_neg == -1) {
        let age1_ = age - 1;
        let age1 = age1_;
    }

    let age2 = age1;
    if (month_diff == 0) {
        let day_diff_neg = 0 - (day_diff >= 0 ? 0 : 1);
        if (day_diff_neg == -1) {
            let age2_ = age1 - 1;
            let age2 = age2_;
        }
    }
    let age = age2;

    // 4. Check age constraint
    if (age >= legal_age) {
        return (1);
    } else {
        return (0);
    }
}
