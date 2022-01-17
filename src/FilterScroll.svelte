<script lang="ts">
    import { onMount } from "svelte";

    export let filterSelector: string;

    let context: CanvasRenderingContext2D;
    let wrap: HTMLDivElement;
    let content: HTMLDivElement;
    let scrollbar: HTMLCanvasElement;
    let scrollbarHovered = false;

    const MIN_THUMB_HEIGHT = 12;

    const fillRect = (rect: DOMRect) =>
        context.fillRect(rect.x, rect.y, rect.width, rect.height);

    const rectContains = (rect: DOMRect, x: number, y: number): boolean => {
        return (
            x >= rect.x &&
            x <= rect.width + rect.x &&
            y >= rect.y &&
            y <= rect.height + rect.y
        );
    };

    const getThumbRect = (): DOMRect => {
        const totalHeight = content.scrollHeight;
        let clientHeight = wrap.clientHeight;

        let thumbHeight =
            Math.min(clientHeight / totalHeight, 1) * clientHeight;

        if (thumbHeight < MIN_THUMB_HEIGHT) {
            clientHeight += thumbHeight - MIN_THUMB_HEIGHT;
            thumbHeight = MIN_THUMB_HEIGHT;
        }

        const scrollY = content.scrollTop;
        let thumbTop = (scrollY * clientHeight) / totalHeight;

        return new DOMRect(
            0,
            Math.round(thumbTop),
            scrollbar.width,
            Math.round(thumbHeight)
        );
    };

    const scrollTo = (y: number) => {
        const totalHeight = content.scrollHeight;
        let clientHeight = wrap.clientHeight;

        let thumbHeight =
            Math.min(clientHeight / totalHeight, 1) * clientHeight;

        if (thumbHeight < MIN_THUMB_HEIGHT) {
            clientHeight += thumbHeight - MIN_THUMB_HEIGHT;
            thumbHeight = MIN_THUMB_HEIGHT;
        }

        content.scrollTop =
            ((y - thumbHeight / 2) / clientHeight) * totalHeight;
    };

    function onMove(e: PointerEvent) {
        scrollbarHovered = rectContains(getThumbRect(), e.offsetX, e.offsetY);
        setTimeout(repaint);
    }

    function onDown(e: PointerEvent) {
        scrollbarHovered = true;
        if (rectContains(getThumbRect(), e.offsetX, e.offsetY)) {
            // start drag
        } else {
            scrollTo(e.pageY);
            // start drag
        }
        setTimeout(repaint);
    }

    function repaint() {
        requestAnimationFrame(() => {
            scrollbar.width = scrollbar.offsetWidth;
            scrollbar.height = scrollbar.offsetHeight;

            // clear canvas
            context.fillStyle = "#FFFFFF";
            context.fillRect(0, 0, scrollbar.width, scrollbar.height);

            const getScaledElementRect = (element: HTMLElement): DOMRect => {
                return new DOMRect(
                    0,
                    Math.round(
                        (element.offsetTop / content.scrollHeight) *
                            wrap.clientHeight
                    ),
                    6,
                    Math.max(
                        Math.round(
                            (element.offsetHeight / content.scrollHeight) *
                                wrap.clientHeight
                        ),
                        1
                    )
                );
            };
            // draw filtered lines
            for (let element of content.querySelectorAll<HTMLElement>(
                filterSelector
            )) {
                context.fillStyle =
                    window.getComputedStyle(element).backgroundColor;
                fillRect(getScaledElementRect(element));
            }

            // thumb
            context.fillStyle = scrollbarHovered
                ? "rgba(0, 0, 0, 0.1)"
                : "rgba(0, 0, 0, 0.2)";
            fillRect(getThumbRect());
        });
    }

    onMount(() => {
        context = scrollbar.getContext("2d", { alpha: false });

        let resizeObserver = new ResizeObserver(repaint);
        resizeObserver.observe(content);
        for (const child of content.children) resizeObserver.observe(child);

        setTimeout(repaint);
    });
</script>

<svelte:window on:resize="{repaint}" />
<div class="outer">
    <div class="wrap" bind:this="{wrap}">
        <div class="content" bind:this="{content}" on:scroll="{repaint}">
            <slot />
        </div>
    </div>
    <canvas
        class="scrollbar"
        bind:this="{scrollbar}"
        on:pointermove="{onMove}"
        on:pointerleave="{onMove}"
        on:pointerdown="{onDown}"></canvas>
</div>

<style>
    :root {
        --scrollbar-width: 16px;
    }

    .outer {
        height: 100%;
        overflow-y: visible !important;
    }

    .wrap {
        overflow: hidden;
        height: 100%;
        position: relative;
        z-index: 1;
        margin-right: var(--scrollbar-width);
    }

    .content {
        height: 100%;
        width: 100%;
        position: relative;
        overflow-y: auto;
        scroll-behavior: unset;
        scrollbar-width: none;
    }

    .content::-webkit-scrollbar {
        display: none;
    }

    .scrollbar {
        position: fixed;
        top: 0;
        right: 0;
        width: var(--scrollbar-width);
        height: 100%;
        z-index: 999;
        image-rendering: -moz-crisp-edges;
        image-rendering: -webkit-crisp-edges;
        image-rendering: pixelated;
        image-rendering: crisp-edges;
    }
</style>
