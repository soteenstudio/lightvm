export declare const sidebarAPIReferences: ({
    text: string;
    collapsed: boolean;
    items: ({
        text: string;
        link: string;
        collapsed?: undefined;
        items?: undefined;
    } | {
        text: string;
        collapsed: boolean;
        items: {
            text: string;
            link: string;
        }[];
        link?: undefined;
    })[];
    link?: undefined;
} | {
    text: string;
    link: string;
    collapsed?: undefined;
    items?: undefined;
})[];
