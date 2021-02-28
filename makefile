dev:
	cd website && pnpm run dev

test:
	cargo test
	
deploy:
	cd website && pnpm run build && vercel --prod