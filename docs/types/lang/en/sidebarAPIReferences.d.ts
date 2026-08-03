export declare const sidebarAPIReferences: (
  | {
      text: string;
      collapsed: boolean;
      items: (
        | {
            text: string;
            link: string;
        }[];
        link?: undefined;
    })[];
} | {
    text: string;
    collapsed: boolean;
    items: {
        text: string;
        link: string;
    }[];
} | {
    text: string;
    link: string;
})[];
