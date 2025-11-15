import { useEffect, useRef } from "react";
import type { SearchResult } from "../types";
import { pickFallbackIcon } from "../utils/fallbackIcon";

export type ResultListProps = {
    results: SearchResult[];
    selectedIndex: number;
    onSelect: (index: number) => void;
    onActivate: (item: SearchResult) => void;
    resolveResultTag: (item: SearchResult) => string;
};

export const ResultList = ({
    results,
    selectedIndex,
    onSelect,
    onActivate,
    resolveResultTag,
}: ResultListProps) => {
    if (results.length === 0) {
        return null;
    }

    const listRef = useRef<HTMLDivElement | null>(null);
    const activeId = results[selectedIndex]?.id;

    useEffect(() => {
        if (!listRef.current || !activeId) {
            return;
        }
        const activeElement = listRef.current.querySelector<HTMLDivElement>(
            `[data-result-id="${activeId}"]`,
        );
        if (activeElement && typeof activeElement.scrollIntoView === "function") {
            activeElement.scrollIntoView({ block: "nearest" });
        }
    }, [activeId]);

    return (
        <div
            ref={listRef}
            className="result-list"
            role="listbox"
            aria-activedescendant={activeId}
        >
            {results.map((item, index) => {
                const isActive = index === selectedIndex;
                const visual = pickFallbackIcon(item);
                return (
                    <div
                        key={item.id}
                        className={isActive ? "result-item active" : "result-item"}
                        role="option"
                        aria-selected={isActive}
                        data-result-id={item.id}
                    >
                        <button
                            type="button"
                            className="result-button"
                            onClick={() => onSelect(index)}
                            onDoubleClick={() => onActivate(item)}
                            onMouseEnter={() => onSelect(index)}
                        >
                            {item.icon ? (
                                <img
                                    src={`data:image/png;base64,${item.icon}`}
                                    className="result-icon"
                                    alt="result icon"
                                />
                            ) : (
                                <div
                                    className="result-icon placeholder"
                                    style={{
                                        background: visual.background,
                                        color: visual.color,
                                    }}
                                >
                                    {visual.glyph}
                                </div>
                            )}
                            <div className="result-meta">
                                <div className="result-title-row">
                                    <span className="result-title">{item.title}</span>
                                    <span className="result-tag">{resolveResultTag(item)}</span>
                                </div>
                                <div className="result-subtitle" title={item.subtitle}>
                                    {item.subtitle}
                                </div>
                            </div>
                            <div className="result-shortcut" aria-hidden="true">
                                {String(index + 1).padStart(2, "0")}
                            </div>
                        </button>
                    </div>
                );
            })}
        </div>
    );
};
