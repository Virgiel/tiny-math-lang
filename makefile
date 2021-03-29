dev:
	cd website && pnpm run dev

test:
	cd tml && cargo test
	
deploy:
	cd website && pnpm run build && vercel --prod