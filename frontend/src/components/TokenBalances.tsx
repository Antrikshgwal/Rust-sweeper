"use client";
import { useState, useEffect, useCallback } from 'react';
import { useWallet } from '@/contexts/WalletContext';
import { ethers } from 'ethers';
import { TokenBalance } from '@/lib/types';

export const TokenBalances = () => {
    const { address } = useWallet();
    const [balances, setBalances] = useState<TokenBalance[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const fetchBalances = useCallback(async () => {
        if (!address) return;

        setLoading(true);
        setError(null);

        try {
            const response = await fetch('http://localhost:3001/scan', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ wallet_address: address }),
            });

            if (!response.ok) throw new Error('Failed to fetch balances');

            const data = await response.json();
            setBalances(data.balances);
        } catch (err: any) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    }, [address]);

    useEffect(() => {
        fetchBalances();
    }, [fetchBalances]);

    if (!address) return null;

    return (
        <div className="w-full max-w-4xl mt-8">
            <div className="flex items-center justify-between mb-4">
                <h2 className="text-2xl font-bold">Token Balances</h2>
                <button
                    onClick={fetchBalances}
                    disabled={loading}
                    className="px-4 py-1.5 text-sm bg-gray-700 rounded hover:bg-gray-600 disabled:opacity-50 flex items-center gap-2"
                >
                    {loading
                        ? <><div className="w-3 h-3 border-2 border-white border-t-transparent rounded-full animate-spin" /> Refreshing...</>
                        : '↻ Refresh'
                    }
                </button>
            </div>

            <div className="bg-gray-800 rounded-lg shadow-lg">
                {error && (
                    <div className="p-4 text-red-400 flex items-center justify-between">
                        <span>Error: {error}</span>
                        <button onClick={fetchBalances} className="text-sm underline">Retry</button>
                    </div>
                )}

                {!error && (
                    <table className="w-full text-left">
                        <thead>
                            <tr className="border-b border-gray-700">
                                <th className="p-4 text-gray-400 font-medium">Token</th>
                                <th className="p-4 text-gray-400 font-medium">Address</th>
                                <th className="p-4 text-gray-400 font-medium text-right">Balance</th>
                            </tr>
                        </thead>
                        <tbody>
                            {balances.length > 0 ? (
                                balances.map((token) => (
                                    <tr key={token.address} className="border-t border-gray-700 hover:bg-gray-750">
                                        <td className="p-4 font-medium">{token.name}</td>
                                        <td className="p-4 text-gray-400 text-sm font-mono">
                                            {token.address.slice(0, 6)}...{token.address.slice(-4)}
                                        </td>
                                        <td className="p-4 text-right tabular-nums">
                                            {ethers.formatUnits(token.balance, token.decimals)}
                                        </td>
                                    </tr>
                                ))
                            ) : (
                                <tr>
                                    <td colSpan={3} className="p-4 text-center text-gray-400">
                                        {loading ? 'Loading...' : 'No token balances found.'}
                                    </td>
                                </tr>
                            )}
                        </tbody>
                    </table>
                )}
            </div>
        </div>
    );
};
