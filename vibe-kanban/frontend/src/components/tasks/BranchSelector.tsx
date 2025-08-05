import { useState, useMemo, useRef } from 'react';
import { Button } from '@/components/ui/button.tsx';
import { ArrowDown, GitBranch as GitBranchIcon, Search } from 'lucide-react';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu.tsx';
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip.tsx';
import { Input } from '@/components/ui/input.tsx';
import type { GitBranch } from 'shared/types.ts';

type Props = {
  branches: GitBranch[];
  selectedBranch: string | null;
  onBranchSelect: (branch: string) => void;
  placeholder?: string;
  className?: string;
  excludeCurrentBranch?: boolean;
};

function BranchSelector({
  branches,
  selectedBranch,
  onBranchSelect,
  placeholder = 'Select a branch',
  className = '',
  excludeCurrentBranch = false,
}: Props) {
  const [branchSearchTerm, setBranchSearchTerm] = useState('');
  const searchInputRef = useRef<HTMLInputElement>(null);

  // Filter branches based on search term and options
  const filteredBranches = useMemo(() => {
    let filtered = branches;

    // Don't filter out current branch, we'll handle it in the UI
    if (branchSearchTerm.trim()) {
      filtered = filtered.filter((branch) =>
        branch.name.toLowerCase().includes(branchSearchTerm.toLowerCase())
      );
    }

    return filtered;
  }, [branches, branchSearchTerm]);

  const displayName = useMemo(() => {
    if (!selectedBranch) return placeholder;

    // For remote branches, show just the branch name without the remote prefix
    if (selectedBranch.includes('/')) {
      const parts = selectedBranch.split('/');
      return parts[parts.length - 1];
    }
    return selectedBranch;
  }, [selectedBranch, placeholder]);

  const handleBranchSelect = (branchName: string) => {
    onBranchSelect(branchName);
    setBranchSearchTerm('');
  };

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button
          variant="outline"
          size="sm"
          className={`w-full justify-between text-xs ${className}`}
        >
          <div className="flex items-center gap-1.5">
            <GitBranchIcon className="h-3 w-3" />
            <span className="truncate">{displayName}</span>
          </div>
          <ArrowDown className="h-3 w-3" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-80">
        <div className="p-2">
          <div className="relative">
            <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
            <Input
              ref={searchInputRef}
              placeholder="Search branches..."
              value={branchSearchTerm}
              onChange={(e) => setBranchSearchTerm(e.target.value)}
              className="pl-8"
              onKeyDown={(e) => {
                // Prevent the dropdown from closing when typing
                e.stopPropagation();
              }}
              autoFocus
            />
          </div>
        </div>
        <DropdownMenuSeparator />
        <div className="max-h-64 overflow-y-auto">
          {filteredBranches.length === 0 ? (
            <div className="p-2 text-sm text-muted-foreground text-center">
              No branches found
            </div>
          ) : (
            filteredBranches.map((branch) => {
              const isCurrentAndExcluded =
                excludeCurrentBranch && branch.is_current;

              const menuItem = (
                <DropdownMenuItem
                  key={branch.name}
                  onClick={() => {
                    if (!isCurrentAndExcluded) {
                      handleBranchSelect(branch.name);
                    }
                  }}
                  disabled={isCurrentAndExcluded}
                  className={`${selectedBranch === branch.name ? 'bg-accent' : ''} ${
                    isCurrentAndExcluded ? 'opacity-50 cursor-not-allowed' : ''
                  }`}
                >
                  <div className="flex items-center justify-between w-full">
                    <span className={branch.is_current ? 'font-medium' : ''}>
                      {branch.name}
                    </span>
                    <div className="flex gap-1">
                      {branch.is_current && (
                        <span className="text-xs bg-green-100 text-green-800 px-1 rounded">
                          current
                        </span>
                      )}
                      {branch.is_remote && (
                        <span className="text-xs bg-blue-100 text-blue-800 px-1 rounded">
                          remote
                        </span>
                      )}
                    </div>
                  </div>
                </DropdownMenuItem>
              );

              if (isCurrentAndExcluded) {
                return (
                  <TooltipProvider key={branch.name}>
                    <Tooltip>
                      <TooltipTrigger asChild>{menuItem}</TooltipTrigger>
                      <TooltipContent>
                        <p>Cannot rebase a branch onto itself</p>
                      </TooltipContent>
                    </Tooltip>
                  </TooltipProvider>
                );
              }

              return menuItem;
            })
          )}
        </div>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

export default BranchSelector;
