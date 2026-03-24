export interface TokenBalance {
  name: string;
  address: string;
  balance: string;
  decimals: number;
}

export interface ApprovalNeeded {
  token_address: string;
  token_name: string;
  spender: string;
  amount: string;
}

export interface SweepResponse {
  to: string;
  calldata: string;
  approvals_needed: ApprovalNeeded[];
}
