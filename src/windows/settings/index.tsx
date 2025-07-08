import { BookOpen, Heart, Settings } from 'lucide-react';
import { CurrentTab, LibraryTab, SettingsTab } from './tabs';
import { ScrollArea } from '@/components/ui/scroll-area';
import { ScrollMask } from '@/components/ui/scroll-mask';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

export default function SettingsWindow() {
  return (
    <div className="h-screen flex flex-col overflow-hidden bg-background">
      <Tabs defaultValue="current" className="p-6 flex-1 h-0 flex space-y-6">
        <TabsList className="grid w-full grid-cols-3">
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
            <CurrentTab />
          </TabsContent>
          <TabsContent value="library" className="space-y-6">
            <LibraryTab />
          </TabsContent>
          <TabsContent value="settings" className="space-y-6">
            <SettingsTab />
          </TabsContent>
          <ScrollMask />
        </ScrollArea>
      </Tabs>
    </div>
  );
}
