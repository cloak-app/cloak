import { Search as SearchIcon } from 'lucide-react';
import { Input } from './input';
import { cn } from '@/lib/utils';
const Search: React.FC<React.ComponentProps<'input'>> = ({
  className,
  ...props
}) => {
  return (
    <div className="relative">
      <SearchIcon className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
      <Input className={cn('pl-10', className)} {...props} />
    </div>
  );
};

export { Search };
