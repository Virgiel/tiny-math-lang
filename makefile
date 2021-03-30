dev:
	cd website && pnpm run dev

test:
	cd tml && cargo test
	
deploy:
	cd tml && cargo install --path . --bin tml --features build-binary && cd ../website && pnpm run build && vercel --prod