set dotenv-load := true

# Usage: just deploy sepolia <Contract> <Args...>
# Example: just deploy sepolia Swap "arg1" "arg2"
deploy network contract *args:
    @echo "Creating {{contract}} on {{network}}..."
    root_dir="$(dirname "{{justfile()}}")"; \
    cd "$root_dir"; \
    set -a; . "$root_dir/.env"; set +a; \
    contract_name="{{contract}}"; contract_name="${contract_name%.sol}"; \
    DRY_RUN=false forge create "contracts/${contract_name}.sol:${contract_name}" \
        --rpc-url $SEPOLIA_RPC_URL \
          --private-key $PRIVATE_KEY \
           --broadcast \
        --constructor-args {{args}}

verify 

