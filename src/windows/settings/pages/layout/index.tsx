import { AppSidebar } from './components/app-sidebar';
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from '@/components/ui/breadcrumb';
import { Separator } from '@/components/ui/separator';
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from '@/components/ui/sidebar';
import { Outlet, useMatches } from 'react-router';

export default function Layout() {
  const matches = useMatches().filter(
    (match) => (match?.handle as { crumb: () => React.ReactNode })?.crumb,
  );

  const breadcrumbs = matches.map((match, index) => {
    const crumb = (match.handle as { crumb: () => React.ReactNode }).crumb();

    return index === matches.length - 1 ? (
      <BreadcrumbItem key={match.pathname}>
        <BreadcrumbPage>{crumb}</BreadcrumbPage>
      </BreadcrumbItem>
    ) : (
      <>
        <BreadcrumbItem key={match.pathname}>
          <BreadcrumbLink href={match.pathname}>{crumb}</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
      </>
    );
  });

  return (
    <SidebarProvider>
      <AppSidebar />
      <SidebarInset>
        <header className="flex h-16 shrink-0 items-center gap-2 border-b px-4">
          <SidebarTrigger className="-ml-1" />
          <Separator
            orientation="vertical"
            className="mr-2 data-[orientation=vertical]:h-4"
          />
          <Breadcrumb>
            <BreadcrumbList>{breadcrumbs}</BreadcrumbList>
          </Breadcrumb>
        </header>
        <div className="flex flex-1 flex-col gap-4 p-4">
          <Outlet />
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}
