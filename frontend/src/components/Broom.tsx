"use client";
import React, { useRef } from 'react';
import { useFrame } from '@react-three/fiber';
import { useGLTF } from '@react-three/drei';

// You can find a 3D model of a broom online, e.g., on Sketchfab
// For this example, I'll assume you have a broom.glb file in your public folder
useGLTF.preload('/broom.glb');

export function Broom(props: any) {
  const group = useRef();
  const { nodes, materials } = useGLTF('/broom.glb') as any;

  useFrame((state, delta) => {
    if (group.current) {
      // Simple animation: make the broom sweep back and forth
      const t = state.clock.getElapsedTime();
      (group.current as any).rotation.z = Math.sin(t * 5) * 0.5;
      (group.current as any).position.x = Math.sin(t * 3) * 0.2;
    }
  });

  return (
    <group ref={group} {...props} dispose={null}>
      {/* This is a placeholder structure. You will need to replace it with your actual model's structure. */}
      <group rotation={[-Math.PI / 2, 0, 0]}>
        <mesh
          castShadow
          receiveShadow
          geometry={nodes.Broom_Handle_0.geometry}
          material={materials.Wood}
        />
        <mesh
          castShadow
          receiveShadow
          geometry={nodes.Broom_Bristles_0.geometry}
          material={materials.Straw}
        />
      </group>
    </group>
  );
}
