"use client";
import { useRef } from 'react';
import { useFrame } from '@react-three/fiber';
import { useGLTF } from '@react-three/drei';
import * as THREE from 'three';

useGLTF.preload('/broom.glb');

export function Broom() {
    const group = useRef<THREE.Group>(null);
    const { scene } = useGLTF('/broom.glb');

    useFrame((state) => {
        if (group.current) {
            const t = state.clock.getElapsedTime();
            group.current.rotation.z = Math.sin(t * 5) * 0.3;
            group.current.position.x = Math.sin(t * 3) * 0.3;
        }
    });

    return (
        <group ref={group} scale={[3, 3, 3]} rotation={[0.3, 0, Math.PI / 4]}>
            <primitive object={scene} />
        </group>
    );
}
