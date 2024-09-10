export const Shortcut = (node, params) => {
    let handler;
    let isWaiting = false;

    const removeHandler = () => window.removeEventListener('keydown', handler);
    const setHandler = () => {
        removeHandler();
        if (!params) return;

        handler = (e) => {
            if (isWaiting) return;
            if ((!!params.alt != e.altKey) ||
                (!!params.shift != e.shiftKey) ||
                (!!params.control != (e.ctrlKey || e.metaKey)) ||
                params.code != e.code) return;

            e.preventDefault();
            params.callback ? params.callback() : node.click();

            isWaiting = true;
            setTimeout(() => {
                isWaiting = false;
            }, params.delay || 200);
            
        };
        window.addEventListener('keydown', handler);
    };
    setHandler();
    return {
        update: setHandler,
        destroy: removeHandler,
    };
};