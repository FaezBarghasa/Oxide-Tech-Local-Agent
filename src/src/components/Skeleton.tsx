import React from 'react';
export const Skeleton: React.FC<{className?:string}> = ({className=''})=><div aria-hidden className={`animate-pulse rounded-md bg-white/[0.06] ${className}`}/>;
export const SkeletonRows: React.FC<{rows?:number}> = ({rows=3})=><div className="flex flex-col gap-2" aria-label="loading">{Array.from({length:rows}).map((_,i)=><Skeleton key={i} className="h-10 w-full"/>)}</div>;
