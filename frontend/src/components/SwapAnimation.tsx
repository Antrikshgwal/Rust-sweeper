"use client";
import React, { useRef } from 'react';
import { useFrame } from '@react-three/fiber';
import { Box } from '@react-three/drei';

export function SwapAnimation(props: any) {
    const group = useRef();

    useFrame((state, delta) => {
        if (group.current) {
            // Simple animation: two boxes rotating around each other
            const t = state.clock.getElapsedTime();
            (group.current as any).rotation.y = t * 2;
        }
    });

    return (
        <group ref={group} {...props} dispose={null}>
            <Box args={[1, 1, 1]} position={[-1.5, 0, 0]}>
                <meshStandardMaterial color="blue" />
            </Box>
            <Box args={[1, 1, 1]} position={[1.5, 0, 0]}>
                <meshStandardMaterial color="green" />
            </Box>
        </group>
    );
}
