import React from 'react';
import { Outlet, useMatches } from 'react-router';
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

export default function Layout() {
  const matches = useMatches().filter(
    (match) => (match?.handle as { crumb: () => string })?.crumb,
  );

  const breadcrumbs = matches.map((match, index) => {
    const crumb = (match.handle as { crumb: () => string }).crumb();

    return (
      <React.Fragment key={crumb}>
        {index === matches.length - 1 ? (
          <BreadcrumbItem>
            <BreadcrumbPage>{crumb}</BreadcrumbPage>
          </BreadcrumbItem>
        ) : (
          <>
            <BreadcrumbItem>
              <BreadcrumbLink href={match.pathname}>{crumb}</BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
          </>
        )}
      </React.Fragment>
    );
  });

  return (
    <SidebarProvider>
      <AppSidebar />
      <SidebarInset className="w-0">
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
