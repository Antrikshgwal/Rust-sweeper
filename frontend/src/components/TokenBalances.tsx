"use client";
import { useState, useEffect } from 'react';
import { useWallet } from '@/contexts/WalletContext';
import { ethers } from 'ethers';
import { TokenBalance } from '@/lib/types';

export const TokenBalances = () => {
    const { address } = useWallet();
    const [balances, setBalances] = useState<TokenBalance[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchBalances = async () => {
            if (!address) return;

            setLoading(true);
            setError(null);

            try {
                const response = await fetch('http://localhost:3001/scan', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({ wallet_address: address }),
                });

                if (!response.ok) {
                    throw new Error('Failed to fetch balances');
                }

                const data = await response.json();
                setBalances(data.balances);
            } catch (err: any) {
                setError(err.message);
            } finally {
                setLoading(false);
            }
        };

        fetchBalances();
    }, [address]);

    if (!address) {
        return null;
    }

    if (loading) {
        return <p>Loading balances...</p>;
    }

    if (error) {
        return <p className="text-red-500">Error: {error}</p>;
    }

    return (
        <div className="w-full max-w-4xl mt-8">
            <h2 className="text-2xl font-bold mb-4">Token Balances</h2>
            <div className="bg-gray-800 rounded-lg shadow-lg">
                <table className="w-full text-left">
                    <thead>
                        <tr>
                            <th className="p-4">Token</th>
                            <th className="p-4">Balance</th>
                        </tr>
                    </thead>
                    <tbody>
                        {balances.length > 0 ? (
                            balances.map((token) => (
                                <tr key={token.address} className="border-t border-gray-700">
                                    <td className="p-4">{token.name}</td>
                                    <td className="p-4">
                                        {ethers.formatUnits(token.balance, token.decimals)}
                                    </td>
                                </tr>
                            ))
                        ) : (
                            <tr>
                                <td colSpan={2} className="p-4 text-center">
                                    No tokens found.
                                </td>
                            </tr>
                        )}
                    </tbody>
                </table>
            </div>
        </div>
    );
};
