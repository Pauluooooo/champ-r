import React, { useCallback, useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api';

export function Rune() {
    const [championId, setChampionId] = useState(0);
    const [championAlias, setChampionAlias] = useState('');
    const [runes, setRunes] = useState<any[]>([]);

    const getRuneList = useCallback(() => {
        if (!championAlias) {
            return;
        }
        
        invoke(`get_runes`, { sourceName: "u.gg", championAlias }).then((ret: any) => {
          console.log(ret);
          setRunes(ret);
        });
      }, [championAlias]);

    useEffect(() => {
        let unlisten: () => any = () => null;
        listen('popup_window::selected_champion', ({ payload }: { payload: any }) => {
            console.log(payload);
            setChampionId(payload.champion_id);
            setChampionAlias(payload.champion_alias);
        }).then(un => {
            unlisten = un;
        });

        return () => {
            unlisten()
        }
    }, []);

    useEffect(() => {
        getRuneList();
    }, [getRuneList]);

    return (
        <section>
            <h1>RUNE WINDOW</h1>
            <p>current champion: <code>{championId}</code> <code>{championAlias}</code></p>
            <button onClick={getRuneList}>Get Rune List</button>

            <pre>
                {JSON.stringify(runes, null, 2)}
            </pre>
        </section>
    )
}
