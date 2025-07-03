import { BookOpen, Heart, Settings } from 'lucide-react';
import { Current, Library, Settings as SettingsTab } from './tabs';
import { WindowTitleBar } from '@/components/titlebar';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

export default function SettingsWindow() {
  return (
    <div className="w-screen h-screen bg-background rounded-lg border flex flex-col overflow-hidden">
      <WindowTitleBar
        className="w-full active:shadow-sm"
        data-tauri-drag-region
      />

      <Tabs defaultValue="current" className="p-6 flex-1 flex space-y-6">
        <TabsList className="grid w-full grid-cols-3 after:">
          <TabsTrigger value="current" className="flex items-center gap-2">
            <BookOpen className="h-4 w-4" />
            当前阅读
          </TabsTrigger>
          <TabsTrigger value="library" className="flex items-center gap-2">
            <Heart className="h-4 w-4" />
            小说列表
          </TabsTrigger>
          <TabsTrigger value="settings" className="flex items-center gap-2">
            <Settings className="h-4 w-4" />
            阅读设置
          </TabsTrigger>
        </TabsList>

        <ScrollArea className="flex-1 h-0">
          <TabsContent value="current" className="space-y-6">
            <Current />
          </TabsContent>
          <TabsContent value="library" className="space-y-6">
            <Library />
          </TabsContent>
          <TabsContent value="settings" className="space-y-6">
            <SettingsTab />
          </TabsContent>
          <div className="bg-white pointer-events-none sticky bottom-0 flex h-40 [mask-image:linear-gradient(transparent,#000000)]" />
        </ScrollArea>
      </Tabs>
    </div>
  );
}
