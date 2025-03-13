import DashboardView from "./dashboard/index.vue";
import WidgetView from "./widget/index.vue";

export type AppView = keyof typeof views;
export const views = {
  main: DashboardView,
  widget: WidgetView
};

export { WidgetView, DashboardView };
