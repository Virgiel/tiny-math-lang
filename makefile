dev:
	cd website && pnpm run dev

test:
	cd tml && cargo test
	
update:
	cd tml && cargo update
	cd website && pnpm update
	
deploy:
	cd tml && cargo install --force --path . --bin tml --features build-binary 
	cd website && pnpm run build && vercel --prod