import DashboardView from "./dashboard/DashboardView.vue";
import WidgetView from "./widget/WidgetView.vue";

export type AppView = keyof typeof views;
export const views = {
  main: DashboardView,
  widget: WidgetView
};

export { WidgetView, DashboardView };
