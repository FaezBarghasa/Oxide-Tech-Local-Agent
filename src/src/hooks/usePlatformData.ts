import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export interface UsePlatformDataOptions<T> {
  ipcCommand: string;
  ipcPayload?: Record<string, unknown>;
  wsEventName?: string;
  refreshIntervalMs?: number;
  defaultData: T;
}

export function usePlatformData<T>({
  ipcCommand,
  ipcPayload,
  wsEventName,
  refreshIntervalMs,
  defaultData,
}: UsePlatformDataOptions<T>) {
  const [data, setData] = useState<T>(defaultData);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchLiveState = async () => {
    try {
      const result = await invoke<T>(ipcCommand, ipcPayload || {});
      setData(result);
      setError(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchLiveState();

    let intervalId: NodeJS.Timeout | null = null;
    if (refreshIntervalMs && refreshIntervalMs > 0) {
      intervalId = setInterval(fetchLiveState, refreshIntervalMs);
    }

    let unlistenFn: (() => void) | null = null;
    if (wsEventName) {
      listen<T>(wsEventName, (event) => {
        setData(event.payload);
        setError(null);
      }).then((unlisten) => {
        unlistenFn = unlisten;
      });
    }

    return () => {
      if (intervalId) clearInterval(intervalId);
      if (unlistenFn) unlistenFn();
    };
  }, [ipcCommand, JSON.stringify(ipcPayload), wsEventName, refreshIntervalMs]);

  return { data, isLoading, error, refetch: fetchLiveState };
}
