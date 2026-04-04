"use client";
import { useState } from 'react';
import { useWallet } from '@/contexts/WalletContext';
import { SweepResponse } from '@/lib/types';
import { ethers } from 'ethers';
import { Canvas } from '@react-three/fiber';
import { Broom } from './Broom';

export const Sweep = () => {
    const { address, signer } = useWallet();
    const [targetToken, setTargetToken] = useState('USDC');
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [sweepPlan, setSweepPlan] = useState<SweepResponse | null>(null);
    const [txHash, setTxHash] = useState<string | null>(null);

    const handleSweep = async () => {
        if (!address || !signer) return;

        setLoading(true);
        setError(null);
        setSweepPlan(null);
        setTxHash(null);

        try {
            // 1. Get Sweep Plan
            const sweepResponse = await fetch('http://localhost:3001/sweep', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ wallet_address: address, target_token: targetToken }),
            });

            if (!sweepResponse.ok) {
                throw new Error('Failed to get sweep plan');
            }

            const plan: SweepResponse = await sweepResponse.json();
            setSweepPlan(plan);

            // 2. Handle Approvals
            for (const approval of plan.approvals_needed) {
                const tokenContract = new ethers.Contract(
                    approval.token_address,
                    ['function approve(address spender, uint256 amount) returns (bool)'],
                    signer
                );
                const approveTx = await tokenContract.approve(approval.spender, approval.amount);
                await approveTx.wait();
            }

            // 3. Sign and Broadcast Sweep Transaction
            const tx = {
                to: plan.to,
                data: plan.calldata,
            };

            const txResponse = await signer.sendTransaction(tx);
            const receipt = await txResponse.wait();
            setTxHash(receipt?.hash ?? null);

        } catch (err: any) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    };

    if (!address) {
        return null;
    }

    return (
        <div className="w-full max-w-4xl mt-8">
            <h2 className="text-2xl font-bold mb-4">Sweep Dust</h2>
            <div className="bg-gray-800 rounded-lg shadow-lg p-6">
                <div className="flex items-center space-x-4">
                    <select
                        value={targetToken}
                        onChange={(e) => setTargetToken(e.target.value)}
                        className="p-2 bg-gray-700 rounded"
                    >
                        <option value="USDC">USDC</option>
                        <option value="USDT">USDT</option>
                        <option value="WETH">WETH</option>
                    </select>
                    <button
                        onClick={handleSweep}
                        disabled={loading}
                        className="px-6 py-2 bg-blue-500 rounded hover:bg-blue-600 disabled:bg-gray-500"
                    >
                        {loading ? 'Sweeping...' : 'Sweep'}
                    </button>
                </div>

                {loading && (
                    <div className="w-full h-64 mt-4">
                        <Canvas camera={{ position: [0, 0, 5], fov: 50 }}>
                            <ambientLight intensity={0.8} />
                            <directionalLight position={[10, 10, 5]} intensity={1} />
                            <Broom />
                        </Canvas>
                    </div>
                )}

                {error && <p className="text-red-500 mt-4">Error: {error}</p>}
                {txHash && (
                    <p className="text-green-500 mt-4">
                        Sweep successful! Tx Hash: {txHash}
                    </p>
                )}
            </div>
        </div>
    );
};
