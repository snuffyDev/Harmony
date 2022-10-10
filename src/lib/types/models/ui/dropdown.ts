export interface DropdownItem {
	id: string;
	name: string;
	action: () => void;
	icon?: string;
}
