"use client";
import { useState } from 'react';
import { useWallet } from '@/contexts/WalletContext';
import { ethers } from 'ethers';

const TOKENS = ['WETH', 'USDC', 'USDT'];
const TOKEN_DECIMALS: Record<string, number> = { WETH: 18, USDC: 6, USDT: 6 };

export const Swap = () => {
    const { address, signer } = useWallet();
    const [fromToken, setFromToken] = useState('WETH');
    const [toToken, setToToken] = useState('USDC');
    const [amount, setAmount] = useState('0.01');
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [txHash, setTxHash] = useState<string | null>(null);

    const handleSwap = async () => {
        if (!address || !signer) return;

        setLoading(true);
        setError(null);
        setTxHash(null);

        try {
            const decimals = TOKEN_DECIMALS[fromToken] ?? 18;
            const swapResponse = await fetch('http://localhost:3001/swap', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    wallet_address: address,
                    token_in: fromToken,
                    token_out: toToken,
                    amount_in: ethers.parseUnits(amount, decimals).toString(),
                }),
            });

            if (!swapResponse.ok) {
                const errorData = await swapResponse.json().catch(() => ({ error: 'Unknown error' }));
                throw new Error(errorData.error || 'Failed to get swap data');
            }

            const swapData = await swapResponse.json();

            // Handle approval if needed
            if (swapData.approval_calldata && swapData.approval_to) {
                const approvalTxResponse = await signer.sendTransaction({
                    to: swapData.approval_to,
                    data: swapData.approval_calldata,
                });
                await approvalTxResponse.wait();
            }

            // Send swap transaction
            const txResponse = await signer.sendTransaction({
                to: swapData.to,
                data: swapData.calldata,
            });
            const receipt = await txResponse.wait();
            setTxHash(receipt?.hash ?? null);

        } catch (err: any) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    };

    if (!address) return null;

    return (
        <div className="w-full max-w-4xl mt-8">
            <h2 className="text-2xl font-bold mb-4">Swap Token</h2>
            <div className="bg-gray-800 rounded-lg shadow-lg p-6">
                <div className="flex items-center space-x-4 flex-wrap gap-y-2">
                    <input
                        type="text"
                        value={amount}
                        onChange={(e) => setAmount(e.target.value)}
                        className="p-2 bg-gray-700 rounded w-28"
                        placeholder="Amount"
                    />
                    <select
                        value={fromToken}
                        onChange={(e) => setFromToken(e.target.value)}
                        className="p-2 bg-gray-700 rounded"
                    >
                        {TOKENS.map(t => <option key={t} value={t}>{t}</option>)}
                    </select>
                    <span className="text-gray-400">→</span>
                    <select
                        value={toToken}
                        onChange={(e) => setToToken(e.target.value)}
                        className="p-2 bg-gray-700 rounded"
                    >
                        {TOKENS.filter(t => t !== fromToken).map(t => <option key={t} value={t}>{t}</option>)}
                    </select>
                    <button
                        onClick={handleSwap}
                        disabled={loading}
                        className="px-6 py-2 bg-green-500 rounded hover:bg-green-600 disabled:bg-gray-500"
                    >
                        {loading ? 'Swapping...' : 'Swap'}
                    </button>
                </div>

                {loading && (
                    <div className="flex items-center mt-4 text-gray-400">
                        <div className="w-5 h-5 border-2 border-green-400 border-t-transparent rounded-full animate-spin mr-3" />
                        Swapping tokens...
                    </div>
                )}

                {error && <p className="text-red-500 mt-4">Error: {error}</p>}
                {txHash && (
                    <p className="text-green-500 mt-4 break-all">
                        Swap successful! Tx: {txHash}
                    </p>
                )}
            </div>
        </div>
    );
};
